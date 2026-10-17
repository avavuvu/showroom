export const holdUnload = (event: BeforeUnloadEvent, hasUnsavedChanges: boolean) => {
    if (!hasUnsavedChanges) {
        return
    }
    event.preventDefault()
    event.returnValue = ""
}
