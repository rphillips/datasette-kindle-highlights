from datasette import hookimpl

@hookimpl
def extra_template_vars(request, view_name):
    return {"q": request.args.get("q") or ""}
