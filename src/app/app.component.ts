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
    let LocalDataDir = await appLocalDataDir()

    if (status === null) {
      localStorage.setItem('serverstatus', JSON.stringify(false))
    } else {
      if (!JSON.parse(status)) {
        await invoke('start_server', {"localDataDir": LocalDataDir})
      }
    }
  }

  ngOnDestroy(): void {
    localStorage.setItem('serverstatus', JSON.stringify(false))
  }
}
