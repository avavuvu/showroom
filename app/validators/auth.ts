import vine, { SimpleMessagesProvider } from '@vinejs/vine'

export const registerValidator = vine.compile(
  vine.object({
    fullName: vine.string().maxLength(100),
    username: vine.string()
      .maxLength(12)
      .regex(/^[a-zA-Z][a-zA-Z0-9-]*$/)
      .uniqueInDatabase({ table: "users", column: "username" }),

    email: vine.string()
      .email()
      .normalizeEmail()
      .uniqueInDatabase({ table: "users", column: "email" }),

    password: vine.string().minLength(8),

  })
)

export const quickRegisterValidator = vine.compile(
  vine.object({
    username: vine.string()
      .maxLength(12)
      .regex(/^[a-zA-Z][a-zA-Z0-9-]*$/)
      .uniqueInDatabase({ table: "users", column: "username" }),

    email: vine.string()
      .email()
      .normalizeEmail()
      .uniqueInDatabase({ table: "users", column: "email" }),
  })
)

export const loginValidator = vine.compile(
  vine.object({
    emailOrUsername: vine.string(),
    password: vine.string(),
  })
)

loginValidator.messagesProvider = new SimpleMessagesProvider({
  required: "Required",
})