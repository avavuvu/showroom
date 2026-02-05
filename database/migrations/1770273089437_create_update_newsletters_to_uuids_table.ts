import { BaseSchema } from '@adonisjs/lucid/schema'

export default class extends BaseSchema {
  protected tableName = 'newsletters'

  async up() {
    this.schema.alterTable(this.tableName, (table) => {
      // 1. Add uuid column
      table.uuid('uuid').defaultTo(this.db.rawQuery('gen_random_uuid()').knexQuery)
    })

    // 2. Populate uuid for existing records (handled by defaultTo above for new records, 
    // but for existing ones we might need a raw query if not using defaultTo with gen_random_uuid).
    // PostgreSQL's gen_random_uuid() in defaultTo should handle existing rows too when adding the column? 
    // Wait, adding a column with default value populates it for existing rows in Postgres.
    // However, to be safe and explicit or if we want to ensure it's not null:

    this.schema.alterTable(this.tableName, (table) => {
        table.uuid('uuid').notNullable().alter()
    })

    // 3. Drop old id
    this.schema.alterTable(this.tableName, (table) => {
      table.dropColumn('id')
    })
    
    // 4. Rename uuid to id and make primary
    this.schema.alterTable(this.tableName, (table) => {
      table.renameColumn('uuid', 'id')
      table.primary(['id'])
    })
  }

  async down() {
    // Reverting is hard because we lost the original integer IDs.
    // We can recreate an integer id column but it will be new IDs.
    this.schema.alterTable(this.tableName, (table) => {
        table.dropPrimary()
        table.renameColumn('id', 'uuid')
    })

    this.schema.alterTable(this.tableName, (table) => {
        table.increments('id').primary()
    })
    
    this.schema.alterTable(this.tableName, (table) => {
        table.dropColumn('uuid')
    })
  }
}