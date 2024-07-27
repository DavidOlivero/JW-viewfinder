import { Injectable, OnDestroy } from '@angular/core';
import { Subject } from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class GetEpubTextService implements OnDestroy {
  private messageSubject = new Subject<any>()
  message$ = this.messageSubject.asObservable()

  constructor() {
    window.addEventListener('message', this.handleMessage.bind(this))
  }

  private handleMessage(event: MessageEvent) {
    if (event.data.type === 'text') {
      this.messageSubject.next(event.data.text)
    }
  }

  ngOnDestroy(): void {
    window.removeEventListener('message', this.handleMessage.bind(this))
  }
}
