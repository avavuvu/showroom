import drive from '@adonisjs/drive/services/main'
import env from '#start/env'
import sharp from 'sharp'
import { MultipartFile } from '@adonisjs/core/types/bodyparser'
import { readFile } from 'fs/promises'

export default class ImageService {
    private static baseUrl = env.get('STORAGE_BASE_URL')

    static getUrl(key: string): string {
        return `${this.baseUrl}/${key}`
    }

    static async storeImage(file: MultipartFile): Promise<{ key: string; url: string }> {
        const key = `images/${Date.now()}-${file.clientName}`
        await file.moveToDisk(key, 's3')
        return { key, url: this.getUrl(key) }
    }

    static async storeProfilePicture(file: MultipartFile, userId: string): Promise<{ key: string; url: string }> {
        const buffer = await readFile(file.tmpPath!)

        const resized = await sharp(buffer)
            .resize(96, 96, { fit: 'cover', position: 'center' })
            .webp({ quality: 85 })
            .toBuffer()

        const key = `avatars/${userId}.webp`

        await drive.use('s3').put(key, resized, {
            contentType: 'image/webp',
            visibility: 'public',
        })

        return { key, url: this.getUrl(key) }
    }
}