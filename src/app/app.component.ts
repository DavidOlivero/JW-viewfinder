import { Component, OnInit, OnDestroy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api";
import { appLocalDataDir, localDataDir } from "@tauri-apps/api/path";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent implements OnInit, OnDestroy {
  async ngOnInit() {
    let status = localStorage.getItem('serverstatus')

    if (status === null) {
      localStorage.setItem('serverstatus', JSON.stringify(false))
    } else {
      if (!JSON.parse(status)) {
        localStorage.setItem('serverstatus', 'true')
        status = 'true'
        await invoke('start_server')
      }
    }
  }

  ngOnDestroy(): void {
    console.log("Destruyendo proceso principal")
    localStorage.setItem('serverstatus', JSON.stringify(false))
  }
}
