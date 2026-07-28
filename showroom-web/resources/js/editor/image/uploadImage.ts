interface SignatureResponse {
    signature: string;
    timestamp: number;
    api_key: string;
    cloud_name: string;
}

interface CloudinaryUploadResponse {
    secure_url: string;
    public_id: string;
}

export interface UploadedImage {
    src: string;
    publicId: string;
}

export async function uploadImage(file: File): Promise<UploadedImage> {
    const signRes = await fetch("/images/sign");
    if (!signRes.ok) throw new Error("Could not get upload signature");
    const { signature, timestamp, api_key, cloud_name } =
        (await signRes.json()) as SignatureResponse;

    const formData = new FormData();
    formData.append("file", file);
    formData.append("api_key", api_key);
    formData.append("timestamp", String(timestamp));
    formData.append("signature", signature);

    const uploadRes = await fetch(
        `https://api.cloudinary.com/v1_1/${cloud_name}/image/upload`,
        { method: "POST", body: formData },
    );

    if (!uploadRes.ok) {
        const body = await uploadRes.json().catch(() => ({}));
        throw new Error(body?.error?.message ?? `Upload failed (${uploadRes.status})`);
    }

    const json = (await uploadRes.json()) as CloudinaryUploadResponse;
    return { src: json.secure_url, publicId: json.public_id };
}
