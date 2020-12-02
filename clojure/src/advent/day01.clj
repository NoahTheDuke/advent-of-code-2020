(ns advent.day01
  (:require [clojure.edn :as edn]
            [clojure.string :as str]))

(defn part1
  [input]
  (let [lines (->> (str/split-lines input)
                   (map #(Integer/parseInt %)))]
    (first (for [n1 lines
                 n2 lines
                 :when (= 2020 (+ n1 n2))]
             (* n1 n2)))))

(defn part2
  [input]
  (let [lines (->> (str/split-lines input)
                   (map #(Integer/parseInt %)))]
    (first (for [n1 lines
                 n2 lines
                 n3 lines
                 :when (= 2020 (+ n1 n2 n3))]
             (* n1 n2 n3)))))
