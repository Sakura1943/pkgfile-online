export const debounce = (fn: () => any, delay: number = 500) => {
    let timer: number |  undefined = undefined
    return function() {
        clearTimeout(timer)
        timer = setTimeout(() => {
            fn()
        }, delay)
    }
}