import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'subscribers'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table.increments('id')

      table.unique(['email', 'user_id'])

      table.integer('user_id').unsigned().references('id').inTable('users').onDelete('CASCADE')
      table.string('email').notNullable()
      table.string('name')
      table.string('token').nullable()
      table.boolean('verified').defaultTo(false)

      table.timestamp('created_at')
      table.timestamp('updated_at')
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}