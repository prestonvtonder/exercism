import kotlin.math.*

object ArmstrongNumber {

    fun check(input: Int): Boolean =
        if (input.toDigits()
            .map { it.pow(input.length()) }
            .sum() == input.toDouble()) true else false

}

fun Int.toDigits(base: Int = 10): List<Double> = sequence {
    var n = this@toDigits
    require(n >= 0)
    while (n > 0) {
        yield(n % base)
        n /= base
    }
}.map(Int::toDouble).toList()

fun Int.length(): Double = when(this) {
    0 -> 1.0
    else -> log10(abs(toDouble())).toInt() + 1.0
}
