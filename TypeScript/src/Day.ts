interface Day<T> {
    parseInput(text: string): T
    solvePart1(input: T): string
    solvePart2(input: T): string
}

export default Day