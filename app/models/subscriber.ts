import { DateTime } from 'luxon'
import User from '#models/user'
import { BaseModel, beforeCreate, belongsTo, column } from '@adonisjs/lucid/orm'
import { randomUUID } from 'crypto'
import type { BelongsTo } from '@adonisjs/lucid/types/relations'

export default class Subscriber extends BaseModel {
  @column({ isPrimary: true })
  declare id: number

  @column()
  declare email: string

  @column()
  declare name: string | null

  @column()
  declare userId: string

  @belongsTo(() => User)
  declare user: BelongsTo<typeof User>

  @beforeCreate()
  static assignUuid(subscriber: Subscriber) {
    subscriber.token = randomUUID()
  }

  @column()
  declare token: string;

  @column()
  declare verified: boolean;

  @column.dateTime({ autoCreate: true })
  declare createdAt: DateTime
}