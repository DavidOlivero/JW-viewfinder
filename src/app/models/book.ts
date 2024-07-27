import { SafeResourceUrl } from "@angular/platform-browser";

export class Book {
    title: string
    coverPath: SafeResourceUrl

    constructor(title: string, coverPath: SafeResourceUrl) {
        this.title = title
        this.coverPath = coverPath
    }
}