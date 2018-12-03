(ns advent-of-code-2015.day4
  (:require [clojure.string :as str]
            [clojure.set :as set]))

(defn md5
  "Generate a md5 checksum for the given string"
  [token]
  (let [hash-bytes
        (doto (java.security.MessageDigest/getInstance "MD5")
          (.reset)
          (.update (.getBytes token)))]
    (.toString
     (new java.math.BigInteger 1 (.digest hash-bytes)) ; Positive and the size of the number
     16)))

(def input "bgvyzdsv")

(def ans1 (loop [cc 1]
            (let [raw (str input cc)
                  h (md5 raw)
                  zeros (- 32 (count h))]
              (if (= zeros 5) cc
                  (recur (inc cc))))))

(def ans2 (loop [cc 1]
            (let [raw (str input cc)
                  h (md5 raw)
                  zeros (- 32 (count h))]
              (if (= zeros 6) cc
                  (recur (inc cc))))))
