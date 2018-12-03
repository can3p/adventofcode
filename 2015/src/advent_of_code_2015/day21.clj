(ns advent-of-code-2015.day21
  (:require [clojure.string :as str]
            [clojure.math.combinatorics :as combo]
            ))

(def input (slurp "resources/day21"))
(def user-hp 100)

(def swords [
             { :name "Dagger" :cost 8 :damage 4 :armor 0  }
             { :name "short" :cost 10 :damage 5 :armor 0  }
             { :name "whammer" :cost 25 :damage 6 :armor 0  }
             { :name "longsw" :cost 40 :damage 7 :armor 0  }
             { :name "greateaxe" :cost 74 :damage 8 :armor 0  }
             ])

(def armors [
             { :name "leather" :cost 13 :damage 0 :armor 1  }
             { :name "chain" :cost 31  :damage 0 :armor 2  }
             { :name "splint" :cost 53 :damage 0 :armor 3  }
             { :name "banded" :cost 75 :damage 0 :armor 4  }
             { :name "plate" :cost 102 :damage 0 :armor 5  }
             ])

(def rings [
            { :name "da1" :cost 25  :damage 1 :armor 0  }
            { :name "da2" :cost 50  :damage 2 :armor 0  }
            { :name "da3" :cost 100 :damage 3 :armor 0  }
            { :name "de1" :cost 20  :damage 0 :armor 1  }
            { :name "de2" :cost 40  :damage 0 :armor 2  }
            { :name "de3" :cost 80 :damage 0 :armor 3  }
             ])

(def equip1 (concat (combo/cartesian-product swords armors)
                    (map list swords)))

(def equip2 (filter #(<= (count %) 2)
                    (combo/subsets rings)))

(def equip (->> (combo/cartesian-product equip1 equip2)
                (map flatten)
                (sort-by #(reduce +
                                  (map :cost %)))))

(def characteristics (map #(reduce (fn [a b] {
                                    :cost (+ (:cost a) (:cost b))
                                    :damage (+ (:damage a) (:damage b))
                                    :armor (+ (:armor a) (:armor b))
                                    :hp user-hp
                                    :chars %
                                    }) { :cost 0 :damage 0 :armor 0 :hp user-hp} %)
                equip))

(def boss-props (->> input
                      (re-seq #"\d+")
                      (map #(Integer/parseInt %))))
(def boss-chars {
                 :hp (nth boss-props 0)
                 :damage (nth boss-props 1)
                 :armor (nth boss-props 2)
                 })

(defn player-wins? [p1 p2]
  (let [
        d1 (max 1 (- (:damage p1) (:armor p2)))
        d2 (max 1 (- (:damage p2) (:armor p1)))
        ]
    (if (<= (quot (:hp p2) d1) (quot (:hp p1) d2))
      true false)))

(def ans1 (:cost (first (filter #(player-wins? % boss-chars) characteristics))))
(def ans2 (:cost (last (filter #(not (player-wins? % boss-chars)) characteristics))))
