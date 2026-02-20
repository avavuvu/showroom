import { DateTime } from 'luxon'
import { BaseModel, beforeCreate, beforeSave, belongsTo, column, hasMany } from '@adonisjs/lucid/orm'
import User from "#models/user"
import string from '@adonisjs/core/helpers/string'
import type { BelongsTo, HasMany } from '@adonisjs/lucid/types/relations'
import { randomUUID } from 'node:crypto'
import Comment from "#models/comment"

export default class Newsletter extends BaseModel {
  public static selfAssignPrimaryKey = true

  @column({ isPrimary: true })
  declare id: string

  @beforeCreate()
  static assignUuid(newsletter: Newsletter) {
    newsletter.id = randomUUID()
  }

  @column()
  declare userId: number

  @belongsTo(() => User)
  declare user: BelongsTo<typeof User>

  @column()
  declare title: string

  @column()
  declare slug: string

  @column()
  declare subtitle: string | null

  @column()
  declare content: string

  @beforeSave()
  static async slugify(newsletter: Newsletter) {
    if (newsletter.$dirty.title && !newsletter.slug) {
      newsletter.slug = string.slug(newsletter.title, { lower: true })
    }
  }

  @column()
  declare sent: boolean

  @hasMany(() => Comment)
  declare comments: HasMany<typeof Comment>

  @column.dateTime({ autoCreate: true })
  declare createdAt: DateTime

  @column.dateTime({ autoCreate: true, autoUpdate: true })
  declare updatedAt: DateTime
}
