pub const PRELUDE: &'static str = "

;; some functions are from:
;; https://en.wikibooks.org/wiki/Write_Yourself_a_Scheme_in_48_Hours/Towards_a_Standard_Library

(define (id x) x)
(define (curry func arg1) (lambda arg (apply func (cons arg1 arg))))
(define (compose f g) (lambda (arg) (f (apply g arg))))

;; folds
(define (foldr func end lst)
  (if (null? lst)
      end
      (func (car lst) (foldr func end (cdr lst)))))

(define (foldl func accum lst)
  (if (null? lst)
      accum
      (foldl func (func accum (car lst)) (cdr lst))))

(define (unfold func init pred)
  (if (pred init)
      (cons init '())
      (cons init (unfold func (func init) pred))))

(define fold foldl)
(define reduce foldr)

;; ??
(define (procedure? x) (eq? (typeof x) 'procedure))
(define (boolean? x) (eq? (typeof x) 'boolean))
(define (char? x) (eq? (typeof x) 'chr))
(define (string? x) (eq? (typeof x) 'str))
(define (integer? x) (eq? (typeof x) 'integer))
(define (inexact? x) (not (exact? x)))
(define (exact? x)
  (define type (typeof x))
  (or (eq? type 'integer)
      (eq? type 'fraction)))
(define (number? x)
  (define type (typeof x))
  (or (eq? type 'integer)
      (eq? type 'fraction)
      (eq? type 'fraction)))
(define (pair? x)
  (define type (typeof x))
  (or (eq? type 'list)
      (eq? type 'list-dotted)))
(define (list? x) (eq? (typeof x) 'list))
(define (output-port? x)
  (define type (typeof x))
  (or (eq? type 'port-std-out)
      (eq? type 'port-binary-out)
      (eq? type 'port-textual-out)))
(define (input-port? x)
  (define type (typeof x))
  (or (eq? type 'port-std-in)
      (eq? type 'port-binary-in)
      (eq? type 'port-textual-in)))
(define (textual-port? x)
  (define type (typeof x))
  (or (eq? type 'port-textual-in)
      (eq? type 'port-textual-out)))
(define (binary-port? x)
  (define type (typeof x))
  (or (eq? type 'port-binary-in)
      (eq? type 'port-binary-out)))

;; booleans
(define (not x) (if x #f #t))

;; numbers
(define zero? (curry = 0))
(define positive? (curry < 0))
(define negative? (curry > 0))
(define (odd? num)  (= (remainder num 2) 1))
(define (even? num) (= (remainder num 2) 0))
(define (abs num) (if (negative? num) (- num) num))
(define (gcd a b) (if (= b 0) (abs a) (gcd b (modulo a b))))
(define (lcm a b) (/ (abs (* a b)) (gcd a b)))
(define (1+ n) (+ n 1))
(define (1- n) (- n 1))

;; lists
(define (list . xs) xs)
(define (null? x) (if (eqv? x '()) #t #f))
(define (sum . lst) (fold + 0 lst))
(define (product . lst) (fold * 1 lst))
(define (map func lst) (foldr (lambda (x y) (cons (func x) y)) '() lst))
(define (filter pred lst) (foldr (lambda (x y) (if (pred x) (cons x y) y)) '() lst))
(define (reverse lst) (fold (flip cons) '() lst))
(define (length lst) (fold (lambda (x y) (+ x 1)) 0 lst))
(define (max first . rest) (fold (lambda (old new) (if (> old new) old new)) first rest))
(define (min first . rest) (fold (lambda (old new) (if (< old new) old new)) first rest))
(define (list-tail lst n) (if (<= n 0) lst (list-tail (cdr lst) (- n 1))))
(define (list-head lst n) (if (<= n 0) '() (cons (car lst) (list-head (cdr lst) (- n 1)))))
(define (list-ref lst n) (car (list-tail lst n)))


(define (mem-helper pred op) (lambda (acc next) (if (and (not acc) (pred (op next))) next acc)))
(define (memq obj lst)       (fold (mem-helper (curry eq? obj) id) #f lst))
(define (memv obj lst)       (fold (mem-helper (curry eqv? obj) id) #f lst))
(define (member obj lst)     (fold (mem-helper (curry equal? obj) id) #f lst))
(define (assq obj alist)     (fold (mem-helper (curry eq? obj) car) #f alist))
(define (assv obj alist)     (fold (mem-helper (curry eqv? obj) car) #f alist))
(define (assoc obj alist)    (fold (mem-helper (curry equal? obj) car) #f alist))

;; string, char
;; FIXME: Should I typecheck?
(define char=? =)
(define char<? <)
(define char>? >)
(define char<=? <=)
(define char>=? >=)

(define (char-ci f a b) (f (char-downcase a) (char-downcase b)))
(define char-ci=? (curry char-ci =))
(define char-ci<? (curry char-ci <))
(define char-ci>? (curry char-ci >))
(define char-ci<=? (curry char-ci <=))
(define char-ci>=? (curry char-ci >=))

(define string=? =)
(define string<? <)
(define string>? >)
(define string<=? <=)
(define string>=? >=)

(define (string-ci f a b) (f (string-downcase a) (string-downcase b)))
(define string-ci=? (curry string-ci =))
(define string-ci<? (curry string-ci <))
(define string-ci>? (curry string-ci >))
(define string-ci<=? (curry string-ci <=))
(define string-ci>=? (curry string-ci >=))

(define substring string-copy)
(define (string-ref s i) (string-copy s i (+ i 1)))
";
