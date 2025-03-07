[es-git](../globals.md) / RemoteRedirect

# 타입 별칭: RemoteRedirect

> **RemoteRedirect**: `"None"` \| `"Initial"` \| `"All"`

Remote redirection settings; whether redirects to another host are
permitted.

By default, git will follow a redirect on the initial request
(`/info/refs`), but not subsequent requests.
