export type LookupResult<T> =
    | { kind: "ok", body: T }
    | { kind: "expired" }
    | { kind: "failed", error: string | null }

const readError = (body: unknown): string | null =>
    body && typeof body === "object" && "error" in body && typeof body.error === "string" ? body.error : null

export const postJson = async <T>(url: string, payload: unknown): Promise<LookupResult<T>> => {
    const response = await fetch(url, {
        method: "POST",
        redirect: "manual",
        headers: { "Content-Type": "application/json", Accept: "application/json" },
        body: JSON.stringify(payload),
    }).catch(() => null)
    if (!response) {
        return { kind: "failed", error: null }
    }
    if (response.type === "opaqueredirect") {
        return { kind: "expired" }
    }
    const body = (await response.json().catch(() => null)) as unknown
    if (response.ok && body) {
        return { kind: "ok", body: body as T }
    }
    return { kind: "failed", error: readError(body) }
}
