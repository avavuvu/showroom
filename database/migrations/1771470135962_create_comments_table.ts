import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'comments'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table.increments('id')
      table.text('content').notNullable()

      table.string('newsletter_id').references('id').inTable('newsletters').onDelete('CASCADE')
      table.string('user_id').references('id').inTable('users').onDelete('CASCADE').nullable()
      table.integer('parent_id').unsigned().references('id').inTable('comments').onDelete('CASCADE').nullable()

      table.timestamp('created_at')
      table.timestamp('updated_at')
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}