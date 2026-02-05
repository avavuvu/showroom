import { FieldContext } from "@vinejs/vine/types";
import db from "@adonisjs/lucid/services/db"
import vine, { VineNumber, VineString } from "@vinejs/vine";

type Options = {
  table: string,
  column: string
}

const uniqueInDatabase = async (value: unknown, options: Options, field: FieldContext) => {
  if (typeof value !== "string" && typeof value !== "number") {
    return
  }

  const result = await db
    .from(options.table)
    .select(options.column)
    .where(options.column, value)
    .first()

    if(result) {
      field.report("This {{ field }} is already taken", "uniqueInDatabase", field)
    }
}

const uniqueInDatabaseRule = vine.createRule(uniqueInDatabase)

declare module "@vinejs/vine" {
  interface VineString {
    uniqueInDatabase(options: Options): this
  }

  interface VineNumber {
    uniqueInDatabase(options: Options): this
  }
}

VineString.macro("uniqueInDatabase", function (this: VineString, options: Options) {
  return this.use(uniqueInDatabaseRule(options))
})

VineNumber.macro("uniqueInDatabase", function (this: VineNumber, options: Options) {
  return this.use(uniqueInDatabaseRule(options))
})
