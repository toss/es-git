[es-git](../globals.md) / RemoteRedirect

# Type Alias: RemoteRedirect

> **RemoteRedirect**: `"None"` \| `"Initial"` \| `"All"`

Remote redirection settings; whether redirects to another host are
permitted.

By default, git will follow a redirect on the initial request
(`/info/refs`), but not subsequent requests.
