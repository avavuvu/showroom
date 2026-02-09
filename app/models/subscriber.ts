import { DateTime } from 'luxon'
import { BaseModel, beforeCreate, column } from '@adonisjs/lucid/orm'
import { randomUUID } from 'crypto'

export default class Subscriber extends BaseModel {
  @column({ isPrimary: true })
  declare id: number

  @column()
  declare email: string

  @column()
  declare userId: number

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