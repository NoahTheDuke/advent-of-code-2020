(ns advent.core
  (:require [clojure.pprint :as pprint]
            [cond-plus.core :refer [cond+]]
            [advent.day01]
            [advent.day02]
            ))

(defn get-day
  [padded-day]
  [(ns-resolve (symbol (str "advent." padded-day)) 'part1)
   (ns-resolve (symbol (str "advent." padded-day)) 'part2)])

(defmacro time2
  [expr]
  `(let [start# (. System (nanoTime))
         ret# ~expr
         elapsed# (/ (double (- (. System (nanoTime)) start#)) 1000000.0)]
     [ret# elapsed#]))

(defn fmt-time
  [ms]
  (cond+
    [(<= ms 1.0)
     (str (Math/round (* 1000.0 ms)) "µs")]
    [(< ms 1000.0)
     (let [whole-ms (Math/floor ms)
           rem-ms (- ms whole-ms)]
       (str whole-ms "ms " (fmt-time rem-ms)))]
    [(< (/ ms 1000.0) 60.0)
     (let [sec (/ ms 1000.0)
           whole-sec (Math/floor sec)
           rem-ms (- ms (* whole-sec 1000.0))]
       (str whole-sec "s " (fmt-time rem-ms)))]
    [:else
     (let [sec (/ ms 1000.0)
           mn (/ sec 60.0)]
       (str (Math/floor mn) "m " (fmt-time (* (rem sec 60.0) 1000.0))))]))

(defn -main
  [& args]
  (let [day (Integer/parseInt (or (first args) "1"))
        padded-day (pprint/cl-format nil "day~2,'0d" day)
        input-file (slurp (str "../inputs/" padded-day ".txt"))
        [part1 part2] (get-day padded-day)]
    (when part1
      (println "Running part 1")
      (let [[result duration] (time2 (part1 input-file))]
        (println (str result ", " (fmt-time duration)))))
    (when part2
      (println "Running part 2")
      (let [[result duration] (time2 (part2 input-file))]
        (println (str result ", " (fmt-time duration)))))))
