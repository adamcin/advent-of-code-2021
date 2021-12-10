import { of } from 'rxjs';

test('the observable emits hello', done => {
  of('hello').subscribe( data => {
    expect(data).toBe('hello');
    done();
  });
});