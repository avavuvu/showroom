import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'users'

  async up() {
    this.schema.createTable(this.tableName, (table) => {
      table.increments('id').notNullable()
      table.string('full_name').nullable()
      table.string('username').notNullable().unique()
      table.string('email', 254).notNullable().unique()
      table.string('password').notNullable()

      table.enum('account_status', ['active', 'inactive', 'suspended'])
        .notNullable()
        .defaultTo('active')

      table.integer('daily_limit').notNullable().defaultTo(100)
      table.integer('bounce_rate').notNullable().defaultTo(0)
      table.integer('complaint_rate').notNullable().defaultTo(0)

      table.timestamp('created_at').notNullable()
      table.timestamp('updated_at').nullable()
    })
  }

  async down() {
    this.schema.dropTable(this.tableName)
  }
}
