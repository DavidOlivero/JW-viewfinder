import { Component, OnInit } from '@angular/core';
import { CommonModule } from "@angular/common";
import { RouterLink } from "@angular/router";
import { ActivatedRoute } from "@angular/router";
import { DomSanitizer, SafeResourceUrl } from "@angular/platform-browser";
import { invoke } from "@tauri-apps/api";
import { GetEpubTextService } from "../services/get-epub-text.service";
import { appLocalDataDir } from "@tauri-apps/api/path";

@Component({
  selector: 'app-viewfinder-book',
  standalone: true,
  imports: [CommonModule, RouterLink],
  templateUrl: './viewfinder-book.component.html',
  styleUrl: './viewfinder-book.component.css'
})
export class ViewfinderBookComponent implements OnInit {
  id: string | undefined
  src?: SafeResourceUrl
  data: string = ''

  async ngOnInit() {
    this.openViewfinderWindow()
    localStorage.setItem('closewindow', JSON.stringify(false))
    this.getEpubText.message$.subscribe((text) => {
      localStorage.setItem('text', JSON.stringify({ text: text, show: false }))
    })
  }

  constructor(private sanitizer: DomSanitizer, private route: ActivatedRoute, private getEpubText: GetEpubTextService) {
    this.data = JSON.parse(localStorage.getItem('text') || '{}')
    this.id = this.route.snapshot.paramMap.get('id')?.replace(/ /g, '_')
    this.getUrl().then(value => this.src = value)
  }

  async getUrl(): Promise<SafeResourceUrl> {
    const folderPath = await appLocalDataDir()

    const bookNav = await invoke<string>("get_nav_page", { epubFolder: folderPath, folderName: this.id })
      .catch(err => alert(err))
    return this.sanitizer.bypassSecurityTrustResourceUrl(`http://127.0.0.1:8080/epub/${this.id}/OEBPS/${bookNav}`)
  }

  async openViewfinderWindow() {
    await invoke("viewfinder")
      .then(err => alert(err))
  }

  showTextInViewFinder() {
    let data = JSON.parse(localStorage.getItem('text') || "{}");
    data.show = true
    localStorage.setItem('text', JSON.stringify(data))
  }

  hideTextInViewFinder() {
    let data = JSON.parse(localStorage.getItem('text') || "{}");
    data.show = null
    localStorage.setItem('text', JSON.stringify(data))
  }

  async closeViewfinderWindow() {
    localStorage.setItem("closewindow", JSON.stringify(true))
  }
}
