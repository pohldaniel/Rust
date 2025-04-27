import {HttpHandlerFn, HttpInterceptorFn, HttpRequest, HttpResponse,} from '@angular/common/http';
import {catchError, throwError} from 'rxjs';

export const intercept: HttpInterceptorFn = (request : HttpRequest<unknown>, next : HttpHandlerFn) => {
  return next(request).pipe(catchError(err => {     
    return throwError(() => err);
  })); 
};