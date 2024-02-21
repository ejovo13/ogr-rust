library(ggplot2)

df <- readr::read_csv("first_2k.csv")

p <- df |> ggplot(aes(id, "Frequency")) + geom_violin() + labs(x="Id")
ggsave("violin.png")

p_hist <- df |> ggplot(aes(id)) + geom_histogram(bins=50, fill="#4F1A14") + labs(x="Id", title="Distribution of the first two thousand Golomb Rulers")
ggsave("histogram.png")

df |> ggplot(aes(order)) + geom_histogram(fill="#4F1A14") + labs(title="Distribution of number of marks in ruler") + theme(text = element_text(size=30))
ggsave("order_hist.png")

df |> ggplot(aes(id, order)) + geom_point() + labs(title="Distribution of number of marks (order)")
ggsave("order_point.png")


file <- "viol.r"
s <- \() {
    source(file)
}

