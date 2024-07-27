import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ViewfinderWindowComponent } from './viewfinder-window.component';

describe('ViewfinderWindowComponent', () => {
  let component: ViewfinderWindowComponent;
  let fixture: ComponentFixture<ViewfinderWindowComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ViewfinderWindowComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ViewfinderWindowComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
