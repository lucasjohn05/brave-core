/* Copyright (c) 2023 The Brave Authors. All rights reserved.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at https://mozilla.org/MPL/2.0/. */

import * as React from 'react'
import Icon from '@brave/leo/react/icon'
import Button from '@brave/leo/react/button'
import Label from '@brave/leo/react/label'
import classnames from '$web-common/classnames'
import formatMessage from '$web-common/formatMessage'
import { getLocale } from '$web-common/locale'
import { useAIChat } from '../../state/ai_chat_context'
import styles from './style.module.scss'

interface PremiumSuggestionProps {
  title?: string
  description?: string
  secondaryActionButton?: React.ReactNode
}

const featuresList = [
  {
    title: getLocale('premiumFeature_1'),
    desc: getLocale('premiumFeature_1_desc'),
    icon: 'widget-generic'
  },
  {
    title: getLocale('premiumFeature_2'),
    desc: getLocale('premiumFeature_2_desc'),
    icon: 'idea'
  },
  {
    title: getLocale('premiumFeature_3'),
    desc: getLocale('premiumFeature_3_desc'),
    icon: 'edit-pencil'
  },
  {
    title: getLocale('premiumFeature_4'),
    desc: getLocale('premiumFeature_4_desc'),
    icon: 'message-bubble-comments'
  }
]

function PremiumSuggestion(props: PremiumSuggestionProps) {
  const aiChatContext = useAIChat()
  const buttonRef = React.useRef<HTMLButtonElement>()

  const pricingInfo = formatMessage(getLocale('premiumPricing'), {
    placeholders: {
      $1: <data>14.99</data>
    }
  })

  const pricingAnnualInfo = formatMessage(getLocale('premiumAnnualPricing'), {
    placeholders: {
      $1: <data>149.99</data>
    }
  })

  React.useEffect(() => {
    if (buttonRef.current === undefined) return
    buttonRef.current.scrollIntoView({ behavior: 'smooth' })
  }, [])

  return (
    <div className={styles.boxPremium}>
      <div className={styles.header}>
        <h4>{props.title}</h4>
        <p>{props.description}</p>
      </div>
      <div className={styles.features}>
        <ul>
          {featuresList.map((entry, i) => {
            return <li key={i}>
              <div className={styles.icon}>
                <Icon name={entry.icon} />
              </div>
              <span>
                {entry.title}
                <p>{entry.desc}</p>
              </span>
            </li>
          })}
        </ul>
      </div>
      {!aiChatContext.isMobile && (
        <div className={styles.priceListWrapper}>
          <div className={styles.priceList}>
            <button className={styles.priceButton} tabIndex={-1}>
              <div className={styles.bestValueColumn}>
                <span className={styles.priceButtonLabel}>{getLocale('oneYearLabel')}</span>
                <Label color='green'>{getLocale('bestValueLabel')}</Label>
              </div>
              <span className={styles.price}>{pricingAnnualInfo}</span>
            </button>
            <button className={classnames(styles.priceButton, styles.priceButtonMonthly)} tabIndex={-1}>
              <span className={styles.priceButtonLabel}>{getLocale('monthlyLabel')}</span>
              <span className={styles.price}>{pricingInfo}</span>
            </button>
          </div>
          <div className={styles.subscriptionPolicy}>
            {getLocale('subscriptionPolicyInfo')}
          </div>
        </div>
      )}
      <div className={styles.actions}>
        <Button onClick={aiChatContext.goPremium} ref={buttonRef}>
          {getLocale('upgradeButtonLabel')}
        </Button>
        {props.secondaryActionButton}
      </div>
    </div>
  )
}

export default PremiumSuggestion
