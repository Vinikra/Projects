import pandas as pd # Importing pandas module
import matplotlib.pyplot as plt # Importing matplotlib module
import seaborn as sns
df = pd.read_csv(r'C:\Users\vinic\Downloads\Life-Expectancy-and-GDP-Starter\all_data.csv') # Reading the csv file and storing it in a variable
print(df.head()) # Printing the data
print(df.Country.unique())
print(df.Year.unique())
df = df.rename({"Life expectancy at birth (years)":"LEABY"}, axis = "columns")
df.head()
plt.figure(figsize=(8,6))
sns.distplot(df.GDP, rug = True, kde=False)
plt.xlabel("GDP in Trillions of U.S. Dollars")
plt.figure(figsize=(8,6))
sns.distplot(df.LEABY, rug = True, kde=False)
plt.xlabel("Life expectancy at birth (years)")
dfMeans = df.drop("Year", axis = 1).groupby("Country").mean().reset_index()
plt.figure(figsize=(8,6))
sns.barplot(x="LEABY", y="Country", data=dfMeans)
plt.xlabel("Life expectancy at birth (years)")
plt.figure(figsize=(8,6))
sns.barplot(x="GDP", y="Country", data=dfMeans)
plt.xlabel("GDP in Trillions of U.S. Dollars")
plt.figure(figsize=(8,6))
sns.lineplot(x=df.Year, y=df.GDP, hue=df.Country)
plt.legend(loc='center left', bbox_to_anchor=(1, 0.5), ncol=1)
plt.ylabel("GDP in Trillions of U.S. Dollars")
plt.show()