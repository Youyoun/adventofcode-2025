import os
import subprocess
from tool.runners.exceptions import CompilationError
from tool.runners.wrapper import SubmissionWrapper


class SubmissionMl(SubmissionWrapper):
    def __init__(self, file: str):
        SubmissionWrapper.__init__(self)
        self.target = file.replace(".ml", ".exe")
        p = subprocess.Popen(
            ["dune", "build", "--verbose", self.target],
            env={
                **os.environ,
            },
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        rc = p.wait()
        if rc > 0:
            raise CompilationError(
                f"Could not compile {file}\nCompiler output:\n{p.stderr.read().decode('utf-8')}"
            )

    def exec(self, input):
        try:
            return subprocess.check_output(
                ["dune", "exec", self.target, input]
            ).decode()
        except OSError as e:
            # subprocess exited with another error
            return RuntimeError(e)
