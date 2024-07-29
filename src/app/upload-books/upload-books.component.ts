import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterLink } from '@angular/router';
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { Book } from "../models/book";
import { DomSanitizer } from "@angular/platform-browser";

@Component({
  selector: 'app-upload-books',
  standalone: true,
  imports: [CommonModule, RouterLink],
  templateUrl: './upload-books.component.html',
  styleUrl: './upload-books.component.css'
})
export class UploadBooksComponent {
  eBooksList: Book[] = []
  isLoaded = false
  aBook = false
  rout = ''

  constructor(private sanitizer: DomSanitizer) {}

  ngOnInit(): void {
    this.eBooksList = JSON.parse(localStorage.getItem('books') || '[]')
  }
  
  async openEpubFile() {
    const epubFile = await open({
      multiple: false,
      filters: [{
        name: 'Epub Files',
        extensions: ['epub']
      }]
    })
    
    if (epubFile) {
      let eBookName = await invoke<string>("get_title", {"epubFile": epubFile})
      let folderPath = eBookName.replace(/ /g, '_')

      if (this.eBooksList.length > 0) {
        let evaluate = true
        this.eBooksList.forEach((book) => {
          if (book.title === eBookName) {
            alert("El libro que quiere subir ya existe")
            evaluate = false
            return
          }
        })

        if (!evaluate) {
          return
        }
      }

      this.aBook = true

      await invoke<boolean>("upload_book", {"epubFile": epubFile, "folderName": folderPath})
          .then((val) => {
            this.isLoaded = val
            this.aBook = false
            
            const imageUrl = 'http://127.0.0.1:8000/resources/' + folderPath + '/cover.png'
            this.eBooksList.push(new Book(eBookName, imageUrl))
            localStorage.setItem("books", JSON.stringify(this.eBooksList))
          })
          .catch(err => alert(err))
    }
  }

  saveIdBook(id: string) {
    localStorage.setItem('idbook', id.replace(/ /g, "_"))
  }

  async openWiewFinder() {
    await invoke("viewfinder")
  }

  deleteLocalStorage() {
    localStorage.clear()
  }
}
