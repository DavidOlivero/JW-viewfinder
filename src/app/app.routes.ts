import { Routes } from "@angular/router";
import { ViewfinderBookComponent } from "./viewfinder-book/viewfinder-book.component";
import { UploadBooksComponent } from "./upload-books/upload-books.component";
import { ViewfinderWindowComponent } from "./viewfinder-window/viewfinder-window.component";

export const routes: Routes = [
    {
        path: '',
        component: UploadBooksComponent
    },
    
    {
        path: 'viewfinder/:id',
        component: ViewfinderBookComponent
    },

    {
        path: 'viewfinderwindow',
        component: ViewfinderWindowComponent
    }
];
