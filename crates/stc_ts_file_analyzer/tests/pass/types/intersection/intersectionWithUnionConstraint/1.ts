// @strict: true

function f2<T extends string | number | undefined, U extends string | null | undefined>(x: T & U) {
    let y1: string | number = x;     // Error
}
