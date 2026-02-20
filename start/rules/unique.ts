import { FieldContext } from "@vinejs/vine/types";
import db from "@adonisjs/lucid/services/db"
import vine, { VineNumber, VineString } from "@vinejs/vine";

type Options = {
  table: string,
  column: string
  exclude?: {
    column: string,
    value: any
  }
}

type OptionsCallback = (field: FieldContext) => Options

const uniqueInDatabase = async (value: unknown, options: Options | OptionsCallback, field: FieldContext) => {
  if (typeof value !== "string" && typeof value !== "number") {
    return
  }

  let resolvedOptions: Options
  if (typeof options === 'function') {
    resolvedOptions = options(field)
  } else {
    resolvedOptions = options
  }

  const query = db
    .from(resolvedOptions.table)
    .select(resolvedOptions.column)
    .where(resolvedOptions.column, value)

  if (resolvedOptions.exclude) {
    query.whereNot(resolvedOptions.exclude.column, resolvedOptions.exclude.value)
  }

  const result = await query.first()

  if (result) {
    field.report("This {{ field }} is already taken", "uniqueInDatabase", field)
  }
}

const uniqueInDatabaseRule = vine.createRule(uniqueInDatabase)

declare module "@vinejs/vine" {
  interface VineString {
    uniqueInDatabase(options: Options | OptionsCallback): this
  }

  interface VineNumber {
    uniqueInDatabase(options: Options | OptionsCallback): this
  }
}

VineString.macro("uniqueInDatabase", function (this: VineString, options: Options | OptionsCallback) {
  return this.use(uniqueInDatabaseRule(options))
})

VineNumber.macro("uniqueInDatabase", function (this: VineNumber, options: Options | OptionsCallback) {
  return this.use(uniqueInDatabaseRule(options))
})
