use super::protocol::{ExecuteStatus, KernelEvent, KernelRequest};
use anyhow::{anyhow, Context, Result};
use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

const PYTHON_KERNEL: &str = r#"
import ast
import contextlib
import io
import json
import sys
import traceback

namespace = {}

def send(event):
    print(json.dumps(event), flush=True)

def execute(cell_id, source):
    stdout_buffer = io.StringIO()
    stderr_buffer = io.StringIO()
    result = None
    status = "success"
    try:
        module = ast.parse(source, mode="exec")
        body = list(module.body)
        last_expr = body[-1] if body and isinstance(body[-1], ast.Expr) else None
        with contextlib.redirect_stdout(stdout_buffer), contextlib.redirect_stderr(stderr_buffer):
            if last_expr is not None:
                prefix = ast.Module(body=body[:-1], type_ignores=[])
                ast.fix_missing_locations(prefix)
                if prefix.body:
                    exec(compile(prefix, "<tupan-cell>", "exec"), namespace, namespace)
                expr = ast.Expression(last_expr.value)
                ast.fix_missing_locations(expr)
                result = eval(compile(expr, "<tupan-cell>", "eval"), namespace, namespace)
            else:
                exec(compile(module, "<tupan-cell>", "exec"), namespace, namespace)
    except BaseException as exc:
        status = "error"
        out = stdout_buffer.getvalue()
        err = stderr_buffer.getvalue()
        if out:
            send({"type": "stdout", "cell_id": cell_id, "text": out})
        if err:
            send({"type": "stderr", "cell_id": cell_id, "text": err})
        send({
            "type": "error",
            "cell_id": cell_id,
            "name": exc.__class__.__name__,
            "message": str(exc),
            "traceback": traceback.format_exc(),
        })
    else:
        out = stdout_buffer.getvalue()
        err = stderr_buffer.getvalue()
        if out:
            send({"type": "stdout", "cell_id": cell_id, "text": out})
        if err:
            send({"type": "stderr", "cell_id": cell_id, "text": err})
        if result is not None:
            send({"type": "execute_result", "cell_id": cell_id, "repr": repr(result)})
    finally:
        send({"type": "execute_done", "cell_id": cell_id, "status": status})

send({"type": "kernel_ready"})

for line in sys.stdin:
    try:
        message = json.loads(line)
        message_type = message.get("type")
        if message_type == "execute_request":
            execute(message["cell_id"], message.get("source", ""))
        elif message_type == "shutdown":
            break
    except BaseException as exc:
        send({
            "type": "error",
            "cell_id": message.get("cell_id", "unknown") if "message" in locals() else "unknown",
            "name": exc.__class__.__name__,
            "message": str(exc),
            "traceback": traceback.format_exc(),
        })
"#;

pub struct PythonProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl PythonProcess {
    pub fn start() -> Result<Self> {
        let mut child = Command::new("python3")
            .arg("-u")
            .arg("-c")
            .arg(PYTHON_KERNEL)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("failed to start python3 kernel process")?;

        let stdin = child.stdin.take().context("failed to open kernel stdin")?;
        let stdout = child
            .stdout
            .take()
            .context("failed to open kernel stdout")?;
        let mut process = Self {
            child,
            stdin,
            stdout: BufReader::new(stdout),
        };

        match process.read_event()? {
            KernelEvent::KernelReady => Ok(process),
            event => Err(anyhow!("unexpected first kernel event: {event:?}")),
        }
    }

    pub fn send(&mut self, request: &KernelRequest) -> Result<()> {
        let line = serde_json::to_string(request).context("failed to serialize kernel request")?;
        writeln!(self.stdin, "{line}").context("failed to write kernel request")?;
        self.stdin.flush().context("failed to flush kernel request")
    }

    pub fn read_event(&mut self) -> Result<KernelEvent> {
        let mut line = String::new();
        let bytes = self
            .stdout
            .read_line(&mut line)
            .context("failed to read kernel event")?;
        if bytes == 0 {
            return Err(anyhow!("python kernel exited"));
        }
        serde_json::from_str(line.trim_end()).context("failed to parse kernel event")
    }

    pub fn execute(&mut self, request: KernelRequest) -> Result<Vec<KernelEvent>> {
        let cell_id = match &request {
            KernelRequest::ExecuteRequest { cell_id, .. } => *cell_id,
            KernelRequest::Shutdown => return Ok(Vec::new()),
        };

        self.send(&request)?;
        let mut events = Vec::new();
        loop {
            let event = self.read_event()?;
            let done = matches!(
                event,
                KernelEvent::ExecuteDone {
                    cell_id: done_cell_id,
                    status: ExecuteStatus::Success | ExecuteStatus::Error,
                } if done_cell_id == cell_id
            );
            events.push(event);
            if done {
                break;
            }
        }
        Ok(events)
    }
}

impl Drop for PythonProcess {
    fn drop(&mut self) {
        let _ = self.send(&KernelRequest::Shutdown);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notebook::ids::CellId;

    #[test]
    fn python_process_keeps_namespace_between_executions() {
        let mut process = PythonProcess::start().expect("kernel should start");

        let first_cell = CellId::new();
        let first_events = process
            .execute(KernelRequest::ExecuteRequest {
                cell_id: first_cell,
                source: "x = 41".to_owned(),
            })
            .expect("first cell should execute");
        assert!(matches!(
            first_events.last(),
            Some(KernelEvent::ExecuteDone {
                status: ExecuteStatus::Success,
                ..
            })
        ));

        let second_cell = CellId::new();
        let second_events = process
            .execute(KernelRequest::ExecuteRequest {
                cell_id: second_cell,
                source: "x + 1".to_owned(),
            })
            .expect("second cell should execute");

        assert!(second_events.iter().any(|event| {
            matches!(
                event,
                KernelEvent::ExecuteResult { cell_id, repr }
                    if *cell_id == second_cell && repr == "42"
            )
        }));
    }
}
