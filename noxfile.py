import nox


@nox.session(tags=["test"])
def tests(session):
    session.install(".", "pytest", "numpy", "scipy")
    session.run("pytest")


@nox.session(tags=["ci"])
def lint(session, fix=False, unsafe=False):
    session.install("ruff")
    command = ["ruff", "check"]
    if fix:
        command.append("--fix")
    if unsafe:
        command.append("--unsafe-fix")
    session.run(*command, ".")


@nox.session
def fmt(session, unsafe=False):
    session.install("ruff")
    command = ["ruff", "format"]
    if unsafe:
        command.append("--unsafe-fix")
    session.run(*command, ".")


@nox.session(tags=["ci"])
def typecheck(session):
    session.install("pyright")
    session.run("pyright")


@nox.session
def docs(session):
    session.install("mkdocs-material")
    session.run("mkdocs", "build")


@nox.session
def prepare(session):
    session.install("pre-commit")
    session.run("pre-commit", "install")
