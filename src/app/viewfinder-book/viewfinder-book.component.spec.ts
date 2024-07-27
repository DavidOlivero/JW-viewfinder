import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ViewfinderBookComponent } from './viewfinder-book.component';

describe('ViewfinderBookComponent', () => {
  let component: ViewfinderBookComponent;
  let fixture: ComponentFixture<ViewfinderBookComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ViewfinderBookComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ViewfinderBookComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
