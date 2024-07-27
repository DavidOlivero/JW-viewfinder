import { TestBed } from '@angular/core/testing';

import { GetEpubTextService } from './services/get-epub-text.service';

describe('GetEpubTextService', () => {
  let service: GetEpubTextService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(GetEpubTextService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
