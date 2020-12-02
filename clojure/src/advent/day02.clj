(ns advent.day02
  (:require [clojure.edn :as edn]
            [clojure.string :as str]
            [cond-plus.core :refer [cond+]]))

(def RE #"(\d+)-(\d+) ([a-z]): ([a-z]+)")

(defn build-rule
  [line]
  (let [[_ low high letter pass] (re-find RE line)]
    {:low (Integer/parseInt low)
     :high (Integer/parseInt high)
     :letter letter
     :pass pass}))

(defn part1
  [input]
  (->> (str/split-lines input)
       (filter (fn [line]
                 (let [rule (build-rule line)
                       count-matches (-> (re-pattern (:letter rule))
                                         (re-seq (:pass rule))
                                         (count))]
                   (<= (:low rule) count-matches (:high rule)))))
       (count)))

(defn part2
  [input]
  (->> (str/split-lines input)
       (filter (fn [line]
                 (let [rule (build-rule line)
                       first-position (str (nth (:pass rule) (dec (:low rule))))
                       second-position (str (nth (:pass rule) (dec (:high rule))))
                       letter (:letter rule)]
                   (cond+
                     [(and (= first-position letter) (= second-position letter))
                      false]
                     [(= first-position letter)]
                     [(= second-position letter)]))))
       (count)))
