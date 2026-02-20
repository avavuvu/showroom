import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'scraps'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table
        .uuid('id')
        .primary()

      table.string("user_id")
        .references("id")
        .inTable("users")
        .notNullable()

      table.boolean("is_successful").notNullable().defaultTo(false)

      table.text("content")

      table.timestamp('created_at')
      table.timestamp('updated_at')
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}