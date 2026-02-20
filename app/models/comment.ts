import { DateTime } from 'luxon'
import { BaseModel, belongsTo, column } from '@adonisjs/lucid/orm'
import Newsletter from "#models/newsletter"
import User from "#models/user"
import type { BelongsTo } from '@adonisjs/lucid/types/relations'

export default class Comment extends BaseModel {
  @column({ isPrimary: true })
  declare id: number

  @column()
  declare content: string

  @column()
  declare parentId: number | null

  @column()
  declare newsletterId: string

  @belongsTo(() => Newsletter)
  declare newsletter: BelongsTo<typeof Newsletter>

  @column()
  declare userId: string | null

  @belongsTo(() => User)
  declare user: BelongsTo<typeof User>

  @column.dateTime({ autoCreate: true })
  declare createdAt: DateTime

  @column.dateTime({ autoCreate: true, autoUpdate: true })
  declare updatedAt: DateTime
}