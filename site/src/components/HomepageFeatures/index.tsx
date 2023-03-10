import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  image: string;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Easy to Use',
    image: require('@site/static/img/runtfile.png').default,
    description: (
      <>
        Runt lets you write your tasks in a simple, easy to read markdown
        file with an easy to understand syntax.
      </>
    ),
  },
  {
    title: 'Autogenerated CLI',
    image: require('@site/static/img/output.png').default,
    description: (
      <>
        Runt automatically gives you a clean CLI interface generated from your
        markdown Runtfile, without the need for you to compile anything.
      </>
    ),
  },
];

function Feature({ title, image, description }: FeatureItem) {
  return (
    <div className={styles.feature}>
      <div>
        <img className={styles.featureImage} src={image} />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className={clsx("row", styles.featureRow)}>
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
