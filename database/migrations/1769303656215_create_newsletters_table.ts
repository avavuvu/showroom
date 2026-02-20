import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'newsletters'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table
        .uuid('id')
        .primary()

      table
        .string('slug')
        .unique()

      table
        .string('subtitle')
        .nullable()


      table.integer("user_id")
        .unsigned()
        .references("id")
        .inTable("users")
        .notNullable()

      table.string("title")

      table.boolean("sent").notNullable().defaultTo(false)

      table.text("content")

      table.timestamp('created_at')
      table.timestamp('updated_at')
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}
