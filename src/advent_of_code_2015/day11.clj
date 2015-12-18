(ns advent-of-code-2015.day11
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(def input "vzbxkghb")

(defn char-range [start end]
  (map char (range (int start) (inc (int end)))))

(def chmap (apply hash-map
                  (flatten (partition 2 1
                           (concat (char-range \a \z)
                                   (list \a))))))

(defn inc-chr [chr] (get chmap chr))

(defn inc-str-seq [sq]
  (let [f (inc-chr (first sq))
        r (if (= f \a f)
            (inc-str-seq (rest sq))
            (rest sq))]
    (conj r f)))

(defn inc-str [s]
  (apply str (-> s
      seq
      reverse
      inc-str-seq
      reverse
      )))

(defn has-seq [s]
  (some? (first (filter (fn [[a b c]]
                (and
                 (not= a \z)
                 (not= b \z)
                 (= (inc-chr a) b)
                 (= (inc-chr b) c)
                 ))
              (partition 3 1
                         (seq s))))))

(defn has-no-bad-letters [s]
  (nil? (re-find #"i|l|o" s)))

(defn has-distinct-pairs [s]
  (->> s
       (str "_")
       (seq)
       (partition 3 1)
       (filter (fn [[a b c]]
                 (and (not= a b c)
                      (= b c))))
       (map (fn [[a b c]] (list b c)))
       (set)
       (count)
       (< 1)
       ))

(defn find-next-pw [input]
  (first
   (drop-while
    #(not ((every-pred has-seq
                       has-no-bad-letters
                       has-distinct-pairs) %))
    (iterate inc-str input))))

(def ans1 (find-next-pw input))
(def ans2 (find-next-pw (inc-str ans1)))

