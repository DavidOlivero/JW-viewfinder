import { Component, OnInit } from '@angular/core';
import { WebviewWindow } from "@tauri-apps/api/window";

@Component({
  selector: 'app-viewfinder-window',
  standalone: true,
  imports: [],
  templateUrl: './viewfinder-window.component.html',
  styleUrl: './viewfinder-window.component.css'
})
export class ViewfinderWindowComponent implements OnInit {
  text = ""
  
  async ngOnInit() {
    window.addEventListener('storage', (event) => {
      if (event.key === 'text') {
        let data = JSON.parse(localStorage.getItem('text') || '{}')
        
        if (data.show) {
          this.text = data.text
          data.show = false
          localStorage.setItem('text', JSON.stringify(data))
        } else if (data.show === null) {
          this.text = ""
          data.show = false
          localStorage.setItem('text', JSON.stringify(data))
        }
      } else if (event.key === 'closewindow') {
        if (JSON.parse(localStorage.getItem('closewindow') || 'false')) {
          const window = WebviewWindow.getByLabel('viewfinder')
          window?.close()
          localStorage.setItem('closewindow', JSON.stringify(false))
        }
      }
    })
  }
}
