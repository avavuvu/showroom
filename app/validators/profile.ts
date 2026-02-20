import vine from '@vinejs/vine'

export const updateProfileValidator = vine.compile(
    vine.object({
        fullName: vine.string().maxLength(100).optional(),
        username: vine.string()
            .maxLength(12)
            .regex(/^[a-zA-Z][a-zA-Z0-9-]*$/)
            .uniqueInDatabase((field) => {
                return {
                    table: 'users',
                    column: 'username',
                    exclude: {
                        column: 'id',
                        value: field.meta.userId,
                    },
                }
            })
            .optional(),
        email: vine.string()
            .email()
            .normalizeEmail()
            .uniqueInDatabase((field) => {
                return {
                    table: 'users',
                    column: 'email',
                    exclude: {
                        column: 'id',
                        value: field.meta.userId,
                    },
                }
            })
            .optional(),
        profileImageUrl: vine.string().optional(),
    })
)
