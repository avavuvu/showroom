import factory from '@adonisjs/lucid/factories'
import Scrap from '#models/newsletter'
import { UserFactory } from './user_factory.js'

export const NewsletterFactory = factory
  .define(Scrap, async ({ faker }) => {
    return {
      title: faker.animal.snake(),
      content: faker.lorem.paragraphs(4, '\n')
    }
  })
  .relation("user", () => UserFactory)
  .build()
