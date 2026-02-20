import { DateTime } from 'luxon'
import { nanoid } from 'nanoid'
import hash from '@adonisjs/core/services/hash'
import { compose } from '@adonisjs/core/helpers'
import { BaseModel, column, hasMany, beforeCreate } from '@adonisjs/lucid/orm'
import { withAuthFinder } from '@adonisjs/auth/mixins/lucid'
import { DbRememberMeTokensProvider } from '@adonisjs/auth/session'
import Subscriber from "#models/subscriber"
import type { HasMany } from '@adonisjs/lucid/types/relations'

const AuthFinder = withAuthFinder(() => hash.use('scrypt'), {
  uids: ['email', 'username'],
  passwordColumnName: 'password',
})

export default class User extends compose(BaseModel, AuthFinder) {
  static rememberMeTokens = DbRememberMeTokensProvider.forModel(User)

  public static selfAssignPrimaryKey = true

  @column({ isPrimary: true })
  declare id: string

  @beforeCreate()
  static assignId(user: User) {
    user.id = nanoid()
  }

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

  static defaultProfileImageUrls = [
    "default_avatars/0.webp",
    "default_avatars/1.webp",
    "default_avatars/2.webp",
    "default_avatars/3.webp",
    "default_avatars/4.webp",
    "default_avatars/5.webp",
    "default_avatars/6.webp",
    "default_avatars/7.webp",
    "default_avatars/8.webp",
    "default_avatars/9.webp",
  ] as const

  @beforeCreate()
  static async assignDefaultProfileImage(user: User) {
    if (!user.profileImageUrl) {
      const randomIndex = Math.floor(Math.random() * User.defaultProfileImageUrls.length)
      user.profileImageUrl = User.defaultProfileImageUrls[randomIndex]
    }
  }
}
