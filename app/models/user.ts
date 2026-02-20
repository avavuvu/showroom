import { DateTime } from 'luxon'
import hash from '@adonisjs/core/services/hash'
import { compose } from '@adonisjs/core/helpers'
import { BaseModel, column, hasMany, beforeCreate } from '@adonisjs/lucid/orm'
import { withAuthFinder } from '@adonisjs/auth/mixins/lucid'
import { DbRememberMeTokensProvider } from '@adonisjs/auth/session'
import Subscriber from './subscriber.js'
import type { HasMany } from '@adonisjs/lucid/types/relations'

const AuthFinder = withAuthFinder(() => hash.use('scrypt'), {
  uids: ['email', 'username'],
  passwordColumnName: 'password',
})

export default class User extends compose(BaseModel, AuthFinder) {
  static rememberMeTokens = DbRememberMeTokensProvider.forModel(User)

  @column({ isPrimary: true })
  declare id: number

  @column()
  declare fullName: string | null

  @column()
  declare username: string

  @column()
  declare email: string

  @column({ serializeAs: null })
  declare password: string

  @column.dateTime({ autoCreate: true })
  declare createdAt: DateTime

  @column.dateTime({ autoCreate: true, autoUpdate: true })
  declare updatedAt: DateTime | null

  @column()
  declare accountStatus: "active" | "inactive" | "suspended"

  @column()
  declare premiumStatus: "free" | "premium"

  @column()
  declare dailyLimit: number

  @column()
  declare bounceRate: number

  @column()
  declare complaintRate: number

  @hasMany(() => Subscriber)
  declare subscribers: HasMany<typeof Subscriber>

  @column()
  declare profileImageUrl: string | null

  @beforeCreate()
  static async assignDefaultProfileImage(user: User) {
    if (!user.profileImageUrl) {
      const defaultProfileImageUrls = [
        "/profilepics/default1.png",
        "/profilepics/default2.png",
        "/profilepics/default3.png",
        "/profilepics/default4.png",
        "/profilepics/default5.png",
        "/profilepics/default6.png",
        "/profilepics/default7.png",
        "/profilepics/default8.png",
        "/profilepics/default9.png",
        "/profilepics/default10.png",
      ] as const

      const randomIndex = Math.floor(Math.random() * defaultProfileImageUrls.length)
      user.profileImageUrl = defaultProfileImageUrls[randomIndex]
    }
  }
}
